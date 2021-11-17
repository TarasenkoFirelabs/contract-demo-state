import * as nearAPI from "near-api-js";
import { makeObservable } from 'mobx';
import { ConnectConfig, Near } from "near-api-js";
import { Config } from "./Config";
import { formatNearAmount } from "near-api-js/lib/utils/format";
import Storage from "./Storage";



const {
    FUNDING_DATA,
    FUNDING_DATA_BACKUP,
    ACCOUNT_LINKS,
    GAS,
    SEED_PHRASE_LOCAL_COPY,
    networkId,
    nodeUrl,
    walletUrl,
    nameSuffix,
    contractName
} = Config;
console.log(networkId);
enum Locals {
    ACCESS_TOKEN = 'access_token',
    REFRESH_TOKEN = 'refresh_token',
    CURRENT_USER_EMAIL = 'current_user_email',
    CURRENT_USER_PHONE = 'current_user_phone'
  }

class Auth extends Storage<Locals> {
    near: null | Near = null;
    wallet: null | nearAPI.WalletConnection = null;
    // networkId: null | ConnectConfig = null;
    // nodeUrl: null | string = null;
    // walletUrl: null | string = null;
    signedIn: boolean = false;
    balance: string = '';

    logged: boolean = false;
    token: string = '';
    currentUser: null | Object = null;
    processing: boolean  = false;

    private static instance?: Auth;

    constructor() {
        super();
        makeObservable(this, {});
    }

    tryToConnect = async () => {
        this.near = await nearAPI.connect({
            networkId, nodeUrl, walletUrl, deps: { keyStore: new nearAPI.keyStores.BrowserLocalStorageKeyStore() },
        });

        // Needed to access wallet
        this.wallet = new nearAPI.WalletConnection(this.near, null);

        this.signedIn = this.wallet.isSignedIn()
        if(this.signedIn) {
            this.balance = formatNearAmount((await this.wallet.account().getAccountBalance()).available, 2)
        }


        // Load in account data
        let currentUser;
        if(walletConnection.getAccountId()) {
            currentUser = {
                accountId: walletConnection.getAccountId(),
                balance: (await walletConnection.account().state()).amount / Math.pow(10, 24)
            };

            currentUser.account = await near.account(currentUser.accountId);
        }

        // Initializing our contract APIs by contract name and configuration
        const contract = await new nearAPI.Contract(walletConnection.account(), nearConfig.contractName, {
            // View methods are read-only – they don't modify the state, but usually return some value
            viewMethods: [],
            // Change methods can modify the state, but you don't receive the returned value when called
            changeMethods: ['call', 'make_wallet', 'log_analytics'],
            // Sender is the account ID to initialize transactions.
            // getAccountId() will return empty string if user is still unauthorized
            sender: walletConnection.getAccountId()
        });



        return this.wallet
    }

    signIn = (successRedirectUrl: string, failedRedirectUrl: string) => {
        if (this.wallet) { 
            this.wallet.requestSignIn(contractName, 'Blah Blah', successRedirectUrl, failedRedirectUrl);
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
