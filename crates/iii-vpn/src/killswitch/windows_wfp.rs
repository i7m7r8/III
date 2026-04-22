use anyhow::{Context, Result};
use windows::Win32::Foundation::*;
use windows::Win32::NetworkManagement::WindowsFilteringPlatform::*;
use windows::Win32::Security::PSECURITY_DESCRIPTOR;
use windows::Win32::Networking::WinSock::*;
use std::net::Ipv4Addr;
use std::str::FromStr;
use tracing::{info, error};

pub async fn enable_wfp_killswitch(relay_ip: &str) -> Result<isize> {
    unsafe {
        let mut engine_handle = HANDLE::default();
        let session = FWPM_SESSION0 {
            flags: FWPM_SESSION_FLAG_DYNAMIC,
            ..Default::default()
        };

        FwpmEngineOpen0(None, RPC_C_AUTHN_WINNT, None, Some(&session), &mut engine_handle)
            .context("FwpmEngineOpen0 failed")?;

        let h_engine = engine_handle;

        // 1. Add Sublayer
        let sublayer_guid = windows::core::GUID::from_u128(0x12345678_1234_1234_1234_123456789012);
        let sublayer = FWPM_SUBLAYER0 {
            subLayerKey: sublayer_guid,
            displayData: FWPM_DISPLAY_DATA0 {
                name: windows::core::w!("III VPN Killswitch Sublayer"),
                description: windows::core::w!("Sublayer for III VPN Killswitch rules"),
            },
            flags: 0,
            weight: 0xFFFF, // High priority
            ..Default::default()
        };

        FwpmSubLayerAdd0(h_engine, &sublayer, PSECURITY_DESCRIPTOR::default())
            .context("FwpmSubLayerAdd0 failed")?;

        // 2. Allow Loopback
        add_allow_loopback(h_engine, sublayer_guid).context("Failed to add allow loopback rule")?;

        // 3. Allow Relay IP
        if let Ok(ip) = Ipv4Addr::from_str(relay_ip) {
            add_allow_ip(h_engine, sublayer_guid, ip).context("Failed to add allow relay IP rule")?;
        }

        // 4. Add "Block All" Filter (Lowest weight in our sublayer)
        let block_filter_guid = windows::core::GUID::from_u128(0x12345678_1234_1234_1234_123456789013);
        let block_filter = FWPM_FILTER0 {
            filterKey: block_filter_guid,
            displayData: FWPM_DISPLAY_DATA0 {
                name: windows::core::w!("III VPN Block All"),
                description: windows::core::w!("Default block rule for killswitch"),
            },
            flags: FWPM_FILTER_FLAG_NONE,
            subLayerKey: sublayer_guid,
            layerKey: FWPM_LAYER_OUTBOUND_TRANSPORT_V4,
            action: FWPM_ACTION0 {
                r#type: FWP_ACTION_BLOCK,
                ..Default::default()
            },
            weight: FWP_VALUE0 {
                r#type: FWP_UINT8,
                Anonymous: FWP_VALUE0_0 { uint8: 0 },
            },
            ..Default::default()
        };

        FwpmFilterAdd0(h_engine, &block_filter, PSECURITY_DESCRIPTOR::default(), None)
            .context("FwpmFilterAdd0 failed (Block All)")?;

        info!("WFP killswitch enabled for relay: {}", relay_ip);
        Ok(h_engine.0 as isize)
    }
}

pub async fn disable_wfp_killswitch(handle: isize) -> Result<()> {
    unsafe {
        let h_engine = HANDLE(handle as _);
        FwpmEngineClose0(h_engine).context("FwpmEngineClose0 failed")?;
        info!("WFP killswitch disabled");
    }
    Ok(())
}

unsafe fn add_allow_loopback(h_engine: HANDLE, sublayer_guid: windows::core::GUID) -> Result<()> {
    let filter_guid = windows::core::GUID::from_u128(0x12345678_1234_1234_1234_123456789014);
    
    let mut condition = FWPM_FILTER_CONDITION0 {
        fieldKey: FWPM_CONDITION_IP_REMOTE_ADDRESS,
        matchType: FWP_MATCH_EQUAL,
        conditionValue: FWP_VALUE0 {
            r#type: FWP_UINT32,
            Anonymous: FWP_VALUE0_0 { uint32: 0x7F000001 }, // 127.0.0.1
        },
        ..Default::default()
    };

    let filter = FWPM_FILTER0 {
        filterKey: filter_guid,
        displayData: FWPM_DISPLAY_DATA0 {
            name: windows::core::w!("III VPN Allow Loopback"),
            description: windows::core::w!("Allow loopback traffic"),
        },
        flags: FWPM_FILTER_FLAG_NONE,
        subLayerKey: sublayer_guid,
        layerKey: FWPM_LAYER_OUTBOUND_TRANSPORT_V4,
        action: FWPM_ACTION0 {
            r#type: FWP_ACTION_PERMIT,
            ..Default::default()
        },
        numFilterConditions: 1,
        filterCondition: &mut condition,
        weight: FWP_VALUE0 {
            r#type: FWP_UINT8,
            Anonymous: FWP_VALUE0_0 { uint8: 100 },
        },
        ..Default::default()
    };

    FwpmFilterAdd0(h_engine, &filter, PSECURITY_DESCRIPTOR::default(), None)
        .context("FwpmFilterAdd0 failed (Allow Loopback)")?;
    Ok(())
}

unsafe fn add_allow_ip(h_engine: HANDLE, sublayer_guid: windows::core::GUID, ip: Ipv4Addr) -> Result<()> {
    let filter_guid = windows::core::GUID::from_u128(0x12345678_1234_1234_1234_123456789015);
    let ip_u32 = u32::from_be_bytes(ip.octets());

    let mut condition = FWPM_FILTER_CONDITION0 {
        fieldKey: FWPM_CONDITION_IP_REMOTE_ADDRESS,
        matchType: FWP_MATCH_EQUAL,
        conditionValue: FWP_VALUE0 {
            r#type: FWP_UINT32,
            Anonymous: FWP_VALUE0_0 { uint32: ip_u32 },
        },
        ..Default::default()
    };

    let filter = FWPM_FILTER0 {
        filterKey: filter_guid,
        displayData: FWPM_DISPLAY_DATA0 {
            name: windows::core::w!("III VPN Allow Relay"),
            description: windows::core::w!("Allow traffic to the SNI relay"),
        },
        flags: FWPM_FILTER_FLAG_NONE,
        subLayerKey: sublayer_guid,
        layerKey: FWPM_LAYER_OUTBOUND_TRANSPORT_V4,
        action: FWPM_ACTION0 {
            r#type: FWP_ACTION_PERMIT,
            ..Default::default()
        },
        numFilterConditions: 1,
        filterCondition: &mut condition,
        weight: FWP_VALUE0 {
            r#type: FWP_UINT8,
            Anonymous: FWP_VALUE0_0 { uint8: 100 },
        },
        ..Default::default()
    };

    FwpmFilterAdd0(h_engine, &filter, PSECURITY_DESCRIPTOR::default(), None)
        .context("FwpmFilterAdd0 failed (Allow Relay IP)")?;
    Ok(())
}
