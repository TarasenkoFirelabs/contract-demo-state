import * as nearAPI from "near-api-js";
import { makeObservable } from 'mobx';
import getContractsConfig from "../full-config";
import { formatNearAmount } from "near-api-js/lib/utils/format";
import { Contract, Near } from "near-api-js";
import {Account} from "near-api-js/lib/account";
import Storage from "./Storage";

enum Locals {
    ACCESS_TOKEN = 'access_token',
    REFRESH_TOKEN = 'refresh_token',
    CURRENT_USER_EMAIL = 'current_user_email',
    CURRENT_USER_PHONE = 'current_user_phone'
  }

export type UserType = {
    accountId?: string,
    balance?: string,
    account?: Account
}

export type NearConfig = {
    networkId: string,
    nodeUrl: string,
    contractName: string,
    walletUrl: string,
    helperUrl: string
}

// @ts-ignore
console.log('process.env.NEXT_PUBLIC_NODE_ENV', process.env.NEXT_PUBLIC_NODE_ENV);

class Auth extends Storage<Locals> {
    near: null | Near = null;
    wallet: null | nearAPI.WalletConnection = null;
    contract: Contract = null;
    signedIn: boolean = false;
    balance: string = '';

    // networkId: null | ConnectConfig = null;
    // nodeUrl: null | string = null;
    // walletUrl: null | string = null;

    logged: boolean = false;
    token: string = '';
    currentUser: null | UserType = null;
    processing: boolean  = false;

    private static instance?: Auth;

    constructor() {
        super();
        makeObservable(this, {});
    }

    private env: string = process.env.NEXT_PUBLIC_NODE_ENV || 'testnet';
    nearConfig = getContractsConfig(this.env);

    tryToConnect = async () => {
        this.near = await nearAPI.connect({
            networkId: this.nearConfig.networkId,
            nodeUrl: this.nearConfig.nodeUrl,
            walletUrl: this.nearConfig.walletUrl,
            deps: { keyStore: new nearAPI.keyStores.BrowserLocalStorageKeyStore() },
        });

        // Needed to access wallet
        this.wallet = new nearAPI.WalletConnection(this.near, null);

        this.signedIn = this.wallet.isSignedIn()
        if (this.signedIn) {
            this.currentUser = {
                accountId: this.wallet.getAccountId(),
                balance: formatNearAmount((await this.wallet.account().getAccountBalance()).available, 2)
            }   //(await walletConnection.account().state()).amount / Math.pow(10, 24)

            this.currentUser.account = await this.near.account(this.currentUser.accountId);

            // Initializing our contract APIs by contract name and configuration
            this.contract = new nearAPI.Contract(this.wallet.account(), this.nearConfig.contractName, {
                // View methods are read-only – they don't modify the state, but usually return some value
                viewMethods: [],
                // Change methods can modify the state, but you don't receive the returned value when called
                changeMethods: ['call', 'make_wallet', 'log_analytics'],
                // Sender is the account ID to initialize transactions.
                // @ts-ignore-next-line
                sender: this.currentUser.accountId
            });
        }

        return this
    }

    signIn = (successRedirectUrl: string, failedRedirectUrl: string) => {
        if (this.wallet) { 
            this.wallet.requestSignIn(this.nearConfig.contractName, 'Blah Blah', successRedirectUrl, failedRedirectUrl);
        }
    }
        
    public clear() {
        this.clearItems([Locals.ACCESS_TOKEN, Locals.REFRESH_TOKEN, Locals.CURRENT_USER_EMAIL, Locals.CURRENT_USER_PHONE]);
    }

    public static getInstance() {
        if (!this.instance) {
          this.instance = new Auth();
        }
        return this.instance;
    }

    public getAccessToken() {
        return this.get(Locals.ACCESS_TOKEN);
    }
    
    public setAccessToken(accessToken: string) {
        this.set(Locals.ACCESS_TOKEN, accessToken);
    }
    
    public getRefreshToken() {
        return this.get(Locals.REFRESH_TOKEN);
    }
    
    public setRefreshToken(refreshToken: string) {
        this.set(Locals.REFRESH_TOKEN, refreshToken);
    }

    public getCurrentUserEmail() {
        return this.get(Locals.CURRENT_USER_EMAIL);
    }
    
    public setCurrentUserEmail(email: string) {
        this.set(Locals.CURRENT_USER_EMAIL, email);
    }
    
    public getCurrentUserPhone() {
        return this.get(Locals.CURRENT_USER_PHONE);
    }
    
    public setCurrentUserPhone(phone: string) {
        this.set(Locals.CURRENT_USER_PHONE, phone);
    }
}

export const AuthStore = new Auth();
