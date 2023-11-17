use cosmwasm_std::testing::{mock_dependencies,  MockApi, MockQuerier, MockStorage};
use cosmwasm_std::{attr, Addr, Empty, OwnedDeps};
use cw_metadata::Metadata;

use badge_hub::error::ContractError;
use badge_hub::state::*;
use badge_hub::{execute, query};
use badges::{Badge, MintRule};

mod utils;

fn setup_test() -> OwnedDeps<MockStorage, MockApi, MockQuerier, Empty> {
    let mut deps = mock_dependencies();

    BADGES.save(
        deps.as_mut().storage,
        1,
        &Badge {
            manager: Addr::unchecked("larry"),
            metadata: Metadata::default(),
            transferrable: true,
            rule: MintRule::ByKeys,
            expiry: Some(12345),
            max_supply: Some(100),
            current_supply: 2,
        },
    )
    .unwrap();

    KEYS.insert(deps.as_mut().storage, (1, "1234abcd")).unwrap();
    KEYS.insert(deps.as_mut().storage, (1, "4321dcba")).unwrap();

    OWNERS.insert(deps.as_mut().storage, (1, "jake")).unwrap();
    OWNERS.insert(deps.as_mut().storage, (1, "pumpkin")).unwrap();

    deps
}

#[test]
fn purging_keys() {
    let mut deps = setup_test();

    // cannot purge when the badge is available
    {
        let err = execute::purge_keys(
            deps.as_mut(),
            utils::mock_env_at_timestamp(10000),
            1,
            None,
        )
        .unwrap_err();
        assert_eq!(err, ContractError::Available);
    }

    // can purge once the badge becomes unavailable
    {
        let res = execute::purge_keys(
            deps.as_mut(),
            utils::mock_env_at_timestamp(99999),
            1,
            None,
        )
        .unwrap();
        assert_eq!(res.messages, vec![]);
        assert_eq!(
            res.attributes,
            vec![
                attr("action", "badges/hub/purge_keys"),
                attr("id", "1"),
                attr("keys_purged", "2"),
            ],
        );

        let res = query::keys(deps.as_ref(), 1, None, None).unwrap();
        assert_eq!(res.keys.len(), 0);
    }

    // purging again should result in no-op
    {
        let res = execute::purge_keys(
            deps.as_mut(),
            utils::mock_env_at_timestamp(99999),
            1,
            None,
        )
        .unwrap();
        assert_eq!(res.messages, vec![]);
        assert_eq!(
            res.attributes,
            vec![
                attr("action", "badges/hub/purge_keys"),
                attr("id", "1"),
                attr("keys_purged", "0"), // no-op
            ],
        );
    }
}

#[test]
fn purging_owners() {
    let mut deps = setup_test();

    // cannot purge when the badge is available
    {
        let err = execute::purge_owners(
            deps.as_mut(),
            utils::mock_env_at_timestamp(10000),
            1,
            None,
        )
        .unwrap_err();
        assert_eq!(err, ContractError::Available);
    }

    // can purge once the badge becomes unavailable
    {
        let res = execute::purge_owners(
            deps.as_mut(),
            utils::mock_env_at_timestamp(99999),
            1,
            None,
        )
        .unwrap();
        assert_eq!(res.messages, vec![]);
        assert_eq!(
            res.attributes,
            vec![
                attr("action", "badges/hub/purge_owners"),
                attr("id", "1"),
                attr("owners_purged", "2"),
            ],
        );

        let res = query::owners(deps.as_ref(), 1, None, None).unwrap();
        assert_eq!(res.owners.len(), 0);
    }

    // purging again should result in no-op
    {
        let res = execute::purge_owners(
            deps.as_mut(),
            utils::mock_env_at_timestamp(99999),
            1,
            None,
        )
        .unwrap();
        assert_eq!(res.messages, vec![]);
        assert_eq!(
            res.attributes,
            vec![
                attr("action", "badges/hub/purge_owners"),
                attr("id", "1"),
                attr("owners_purged", "0"), // no-op
            ],
        );
    }
}