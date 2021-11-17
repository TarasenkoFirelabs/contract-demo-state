// import { BN } from "bn.js"

// testnet / default
const devConfig = {
    SEED_PHRASE_LOCAL_COPY: '__SEED_PHRASE_LOCAL_COPY',
    FUNDING_DATA: '__FUNDING_DATA',
    ACCOUNT_LINKS: '__ACCOUNT_LINKS',
    GAS: '200000000000000',
    networkId: 'testnet',
    nodeUrl: 'https://rpc.testnet.near.org',
    walletUrl: 'https://wallet.testnet.near.org',
    nameSuffix: '.testnet',
    contractName: 'testnet',
}

const prodConfig = {
    SEED_PHRASE_LOCAL_COPY: '__SEED_PHRASE_LOCAL_COPY',
    FUNDING_DATA: '__FUNDING_DATA',
    ACCOUNT_LINKS: '__ACCOUNT_LINKS',
    GAS: '200000000000000',
    networkId: 'mainnet',
    nodeUrl: 'https://rpc.mainnet.near.org',
    walletUrl: 'https://wallet.near.org',
    nameSuffix: '.near',
    contractName: 'near',
}

export const Config = process.env.NEXT_PUBLIC_ENV === 'dev' ? devConfig : prodConfig
