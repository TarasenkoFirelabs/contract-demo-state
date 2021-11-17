import { Near } from "near-api-js";

class Account {
    near: null | Near = null;

    public setNear(near: Near) {
        this.set(Locals.CURRENT_USER_PHONE, phone);
    }


    isAccountTaken = async (accountId) => {
        if (!this.near) {
            return false;
        }

        const account = new nearAPI.Account(this.near.connection, accountId);
        try {
            await account.state()

        } catch (e: any) {
            console.warn(e.toString())
            if (/does not exist while viewing/.test(e)) {
                return false
            }
        }
        return true
    }
}

export default Account;