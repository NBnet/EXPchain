# `exp dev`

A powerful and convenient development tool for managing local clusters of ETH2-based networks.

Through a `exp dev -h` we can see:

```
Manage development clusters on a local host

Usage: exp dev [OPTIONS] [COMMAND]

Commands:
  create              Create a new ENV
  deposit, -d         Proof-of-Stake, deposit
  validator-exit, -D  Proof-of-Stake, exit all validators on the target node[s]
  destroy             Destroy an existing ENV
  protect             Protect an existing ENV
  unprotect           Unprotect an existing ENV
  start               Start nodes of an existing ENV
  stop                Stop nodes of an existing ENV
  restart             Restart nodes of an existing ENV
  push-nodes          Push some new nodes to an existing ENV
  kick-nodes          Remove(destroy) some nodes from an existing ENV
  switch-EL-to-geth   Switch the EL client to `geth`,
                      NOTE: the node will be left stopped, a `start` operation may be needed
  switch-EL-to-reth   Switch the EL client to `reth`,
                      NOTE: the node will be left stopped, a `start` operation may be needed
  show                Default operation, show the information of an existing ENV
  debug-failed-nodes  Show failed nodes in a list
  list-rpcs           List various RPC endpoints of the ENV
  list                Show names of all existing ENVs
```

#### Management of a single cluster

In the simplest scenario, we can:
- create and start a single cluster(`exp dev create`)
- stop the cluster(`exp dev stop`)
- restart/start the cluster(`exp dev start`)
- destroy the cluster(`exp dev destroy`)

However, `exp dev` can do far more than these, let's show a typical usage flow.

The first step, a new cluster(aka ENV) need to be created by `exp dev create`, all configurations use default values, for example, the name of this ENV will be 'DEFAULT', the listening address of all nodes is '127.0.0.1', and so on.

Now you can do some tests on this new ENV:
- `curl 'http://127.0.0.1:5052/eth/v1/node/identity'`
- `curl -X POST -H 'Content-Type: application/json' -d '{"jsonrpc":"2.0","method":"admin_nodeInfo","params":[],"id":1}' http://127.0.0.1:8545`
- transfer tokens
- deploy and call your contracts
- do deposit operations
- ...

But wait,
- where to get the rpc endpoints?
- where to get tokens?
- where to get the web-service ports of a target ENV?
- where to find existing validator keys?
- ...

In a word, how to easily get all necessary information?

Don't worry, a `exp dev show -e <ENV>` will show you everything you need, you can use a shorter style `exp dev` when using the default ENV, they are equal.

Below is the information of the default ENV, its name is 'DEFAULT'.
`exp dev`:
```json
{
  "is_protected": false,
  "meta": {
    "block_time_in_seconds": 2,
    "custom_data": {
      "cl_bin": "lighthouse",
      "el_geth_bin": "geth",
      "el_reth_bin": "reth"
    },
    "deposit_contract_addr": "0x4242424242424242424242424242424242424242",
    "env_home": "/tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/DEFAULT",
    "env_name": "DEFAULT",
    "fuhrer_nodes": {
      "0": {
        "custom_data": {
          "deposits": {
            "giant issue aisle success illegal bike spike question tent bar rely arctic volcano long crawl hungry vocal artwork sniff fantasy very lucky have athlete": 64
          },
          "el_kind": "Geth"
        },
        "id": 0,
        "kind": "Fuhrer",
        "node_home": "/tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/DEFAULT/0",
        "ports": {
          "cl_bn_metric": 5054,
          "cl_bn_rpc": 5052,
          "cl_discovery": 9000,
          "cl_discovery_quic": 9001,
          "cl_vc_metric": 5064,
          "cl_vc_rpc": 5062,
          "el_discovery": 30303,
          "el_discovery_v5": 9200,
          "el_engine_api": 8551,
          "el_metric": 6060,
          "el_rpc": 8545,
          "el_rpc_ws": 8546
        }
      }
    },
    "genesis_pre_settings": "",
    "host_ip": "127.0.0.1",
    "nodes": {
      "1": {
        "custom_data": {
          "deposits": {
            "cluster forest hold opera essence laugh stadium spend guitar media seat unlock odor horse soap strike afraid satisfy bike salmon royal client universe dutch": 2
          },
          "el_kind": "Geth"
        },
        "id": 1,
        "kind": "ArchiveNode",
        "node_home": "/tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/DEFAULT/1",
        "ports": {
          "cl_bn_metric": 49135,
          "cl_bn_rpc": 27372,
          "cl_discovery": 43000,
          "cl_discovery_quic": 24661,
          "cl_vc_metric": 52422,
          "cl_vc_rpc": 22524,
          "el_discovery": 51277,
          "el_discovery_v5": 21075,
          "el_engine_api": 34201,
          "el_metric": 23192,
          "el_rpc": 48796,
          "el_rpc_ws": 21447
        }
      },
      "2": {
        "custom_data": {
          "deposits": {
            "poem now ribbon piano mistake nerve tribe outer beyond what robot orphan tuition that culture reduce soul cargo iron cup old drip auto claim": 5,
            "scrap zebra settle melt egg hole legend author lazy load scorpion sell detect casino mansion tent twist exact photo version gift cream few uncover": 2
          },
          "el_kind": "Geth"
        },
        "id": 2,
        "kind": "ArchiveNode",
        "node_home": "/tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/DEFAULT/2",
        "ports": {
          "cl_bn_metric": 47909,
          "cl_bn_rpc": 46205,
          "cl_discovery": 59575,
          "cl_discovery_quic": 50563,
          "cl_vc_metric": 31657,
          "cl_vc_rpc": 23876,
          "el_discovery": 27307,
          "el_discovery_v5": 31359,
          "el_engine_api": 65321,
          "el_metric": 42214,
          "el_rpc": 65152,
          "el_rpc_ws": 40657
        }
      }
    },
    "premined_accounts": {
      "0x8943545177806ED17B9F23F0a21ee5948eCaa776": {
        "balance": "1000000000000000000000000000",
        "secretKey": "0xbcdf20249abf0ed6d944c0288fad489e33f66b3960d9e6229c1cd214ed3bbe31"
      }
    }
  }
}
```

As you see in the outputs of `exp dev show`, the nodes in the `fuhrer_nodes` list are some special instances created along with the ENV's birth, and can not be removed, the only way to destroy them is distroy the entire ENV. In contrast, the nodes in the `nodes` list are so-called 'Ordinary Node', they can be removd as your will.

The `premined_accounts` fild hold all test tokens for you.

You can use `exp dev list-rpcs -w` to get all Web3 endpoints.

The initial validators are managed by the first fuhrer node, so all the keys are stored in its home directory. In the above example, it is `/tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/DEFAULT/0/cl/vc/`.

You can pause the ENV by `exp dev stop`, and resume it by `exp dev start` at any time; you can also scale up the ENV by `exp dev push-node`, and scale it down by `exp dev kick-node`.

At last, if you don't need this ENV anymore, you can permanently destroy it with the `exp dev destroy` subcommand.

The above is the simplest management process of a local development environment, which is enough for developers to self-debug on their localhosts.

But obviously, for example, for the scenario of front-end and back-end joint debugging, the simplest ENV configuration above is not enough, so we need some additional configuration options to meet these requirements. Most of these additional configurations need to be specified during the ENV creation process, that is, specified in the scope of the `exp dev create` subcommand.

Let's check the help information of `exp dev create`:
```
Create a new ENV

Usage: exp dev create [OPTIONS]

Options:
  -e, --env-name <ENV_NAME>

  -H, --host-ip <HOST_IP>
          Usually need not to specify [default: 127.0.0.1]
  -t, --block-time-secs <BLOCK_TIME_SECS>
          If not set, use the default value in the genesis,
          has no effect when the `--genesis-data-pre-created` option is specified
  -g, --genesis-custom-settings-path <GENESIS_CUSTOM_SETTINGS_PATH>
          The path of a cfg file in the form of
          'https://github.com/rust-util-collections/EGG/blob/master/defaults.env',
          has no effect when the `--genesis-data-pre-created` option is specified
  -G, --genesis-data-pre-created <GENESIS_DATA_PRE_CREATED>
          Concated paths for specifying the pre-created genesis.tar.gz and vcdata.tar.gz,
          they are usally created by the `make build` of 'https://github.com/rust-util-collections/EGG',
          value format: '/PATH/TO/genesis.tar.gz+/PATH/TO/vcdata.tar.gz',
          the `+` is the delimiter between them two
      --el-geth-bin <EL_GETH_BIN>
          The path of your custom geth binary
      --el-reth-bin <EL_RETH_BIN>
          The path of your custom reth binary
      --cl-bin <CL_BIN>
          The path of your custom lighthouse binary
      --force
          Try to destroy the target ENV and then recreate it
```

For the issue of remote joint debugging, we can use the `--host-ip` option to specify the listening address of the target ENV.

Below is a more complete example with richer options:
```shell
exp dev create \
    -H 192.168.2.5 \
    -e MyEnv \
    -t 10 \
    -N 6 \
    --cl-bin /tmp/lighthouse-x.x.x \
    --el-geth-bin /tmp/geth-x.x.x \
    --el-reth-bin /tmp/reth-x.x.x \
    --force \
```

- All nodes of this ENV will listen on '192.168.2.5'
  - Now you can send this IP to your frondend engineers, the joint debugging will be ok
- The name of this ENV is 'MyEnv'
- The block interval will be 10s
- The number of initial validator nodes is 6
- Use custom cl/el client binaries
- 'force create'
    - That is, if any existing ENV has the same name, it will be destroyed

If you want to check what happened behind the `start`/`stop`/`destroy` etc., check the `mgmt.log` for details.
For example, you want to check the node with ID 2:
```shell
# cat /tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/mgmt.log

[ 2024-10-15 11:23:53 +00:00:00 ]

echo "2443ecbca98c5597f10bffe784be68d772689081d85928dcdee2cfb195d11770" > /tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/auth.jwt | tr -d '\n' || exit 1

if [ ! -d /tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/genesis ]; then
    tar -C /tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2 -xpf /tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/genesis.tar.gz || exit 1
    if [ ! -d /tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/genesis ]; then
        mv /tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/$(tar -tf /tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/genesis.tar.gz | head -1) \
        /tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/genesis \
        || exit 1
    fi
fi

if [ ! -d /tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/el ]; then
    mkdir -p /tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/el || exit 1
    (which geth; geth version; echo) \
    >>/tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/el/el.log 2>&1
    geth init --datadir=/tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/el --state.scheme=hash \
        /tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/genesis/genesis.json \
        >>/tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/el/el.log 2>&1 \
        || exit 1
fi
(which geth; geth version; echo) >>/tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/el/el.log 2>&1

nohup geth \
    --syncmode=full \
    --gcmode=archive \
    --networkid=$(grep -Po '(?<="chainId":)\s*\d+' /tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/genesis/genesis.json | tr -d ' ') \
    --datadir=/tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/el \
    --state.scheme=hash \
    --nat=extip:127.0.0.1 \
    --port=57691 \
    --discovery.port=57691 \
    --discovery.v5 \
    --http --http.addr=127.0.0.1 --http.port=54474 --http.vhosts='*' --http.corsdomain='*' \
    --http.api='admin,debug,eth,net,txpool,web3,rpc' \
    --ws --ws.addr=127.0.0.1 --ws.port=41960 --ws.origins='*' \
    --ws.api='admin,debug,eth,net,txpool,web3,rpc' \
    --authrpc.addr=127.0.0.1 --authrpc.port=62103 \
    --authrpc.jwtsecret=/tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/auth.jwt \
    --metrics \
    --metrics.port=37648  \
    --bootnodes='enode://b4fbaa4be939f54cc37ab6900d7cac91544bef15247442e41b20d2c9c25ca006acb145a7c16697dc170db5b5080cf05c61dcce5da3c4bc4766d7b6c99ad717f1@127.0.0.1:58860' \
    >>/tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/el/el.log 2>&1 &


mkdir -p /tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/cl/bn || exit 1
sleep 0.5

(which lighthouse; lighthouse --version; echo) >>/tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/cl/bn/cl.bn.log 2>&1

nohup lighthouse beacon_node \
    --testnet-dir=/tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/genesis \
    --datadir=/tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/cl/bn \
    --staking \
    --slots-per-restore-point=32 \
    --enr-address=127.0.0.1 \
    --disable-enr-auto-update \
    --disable-upnp \
    --listen-address=127.0.0.1 \
    --port=58473 \
    --discovery-port=58473 \
    --quic-port=56182 \
    --execution-endpoints='http://127.0.0.1:62103' \
    --jwt-secrets=/tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/auth.jwt \
    --suggested-fee-recipient=0x47102e476Bb96e616756ea7701C227547080Ea48 \
    --http --http-address=127.0.0.1 \
    --http-port=37050 --http-allow-origin='*' \
    --metrics --metrics-address=127.0.0.1 \
    --metrics-port=32359 \
    --metrics-allow-origin='*' \
    --boot-nodes='enr:-MK4QOA1U3DygoJ3nreSDRON8kVHWDgietvN9138K2NRjO1bdh0kpogiM-3PiFKOgq7cxuoPd2VYhpxvR9L1curTWOgBh2F0dG5ldHOIAAAAAAAAAACDY3NjBIRldGgykBi3zF9gAAAAAADkC1QCAACCaWSCdjSCaXCEfwAAAYRxdWljguLiiXNlY3AyNTZrMaECQeMfPX2cxqNSB9c-aaXXc25swsy5kNMfuexWOLrSjUSIc3luY25ldHMAg3RjcILQBw' \
    --trusted-peers='16Uiu2HAkyrsM9hvi3UrKRniujkXKeWw2wcjQNMCXLQ28YVTDMbpw' \
    --checkpoint-sync-url=http://127.0.0.1:45384 \
    --enable-private-discovery \
    >>/tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/cl/bn/cl.bn.log 2>&1 &

mkdir -p /tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/cl/vc || exit 1
sleep 1

nohup lighthouse validator_client \
    --testnet-dir=/tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/genesis \
    --datadir=/tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/cl/vc\
    --beacon-nodes='http://127.0.0.1:37050' \
    --init-slashing-protection \
    --suggested-fee-recipient=0x47102e476Bb96e616756ea7701C227547080Ea48 \
    --unencrypted-http-transport \
    --http --http-address=127.0.0.1 \
    --http-port=40933 --http-allow-origin='*' \
    --metrics --metrics-address=127.0.0.1 \
    --metrics-port=57629 --metrics-allow-origin='*' \
     >>/tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}+${USER}/__DEV__/envs/MyEnv/2/cl/vc/cl.vc.log 2>&1 &
```

#### Management of multiple ENVs

Since each ENV can specify its own binaries(lighthouse/reth/geth), the multi-ENV mode is of great significance for functional comparison, testing and problem debugging between different versions or between different features.

Managing multiple ENVs, or in other words, managing custom ENVs is not much different from the default ENV because resource allocation and process running between different ENVs are completely isolated in `exp dev`.

The only difference is that you do not have to explicitly specify the env name when managing the default ENV, but for non-default ENVs, all operations must explicitly specify the name of the target env.

For example, for the default ENV, `exp dev stop` is equal to `exp dev stop -e DEFAULT`, both styles are ok; but there is only one style for a custom ENV, that is `exp dev stop -e YourCustomEnv`.

Also, there are some subcommands designed specifically for multi-ENV management:
- `exp dev list`, list the names of all existing ENVs
- `exp dev show-all`, list the details of all existing ENVs
- `exp dev destroy-all`, destroy all existing ENVs

#### Internal organization of data and logs

All data and logs are located under `/tmp/__CHAIN_DEV__`, so you should have a `/tmp` that is not too small.

Let's check their structures:
```
# tree -F -L 2 /tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}_${USER}/__DEV__/
/tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}_${USER}/__DEV__/
├── envs/               # existing ENVs
│   ├── DEFAULT/        # the default ENV
│   ├── env_A/          # a custom ENV named 'env_A'
│   └── env_B/          # a custom ENV named 'env_B'
└── ports_cache         # allocated ports
```

Let's check the inner structure of 'DEFAULT':
```
/tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}_${USER}/__DEV__/envs/DEFAULT/
├── 0/           # the original node of this ENV, can *not* be removed dynamicly
├── 1/           # the first ordinary node of this ENV, can be removed dynamicly
├── 2/           # the second ordinary node of this ENV, can be removed dynamicly
├── 3/           # ...
└── CONFIG       # config file of this ENV
```

Then further check the internal structure of a node:
```
/tmp/__CHAIN_DEV__/beacon_based/EXPCHAIN/${HOST}_${USER}/__DEV__/envs/MyEnv/1/
├── auth.jwt                # the jwt used to build connection between cl and el
├── cl/
│   ├── bn/
│   │   └── cl.bn.log       # log of the cl beancon process
│   └── vc/
│       ├── cl.vc.log       # log of the cl validator process
│       └── validators/
├── el/
│   └── el.log              # log of the el process
├── genesis/
│   ├── config.yaml         # network core config
│   ├── genesis.json        # genesis file of the el
│   └── genesis.ssz         # genesis file of the cl
├── genesis.tar.gz          # a tar.gz package of the genesis dir
└── mgmt.log                # management log of the `exp` system
```

The `exp` management operations of `exp dev` will be logged in the `mgmt.log` file.

#### OS compatibility

In theory, it can run well on most linux distributions and macOS.
