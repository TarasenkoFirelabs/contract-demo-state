import React, { useCallback, useEffect, useState } from "react";
import { useAlert } from "react-alert";
import styles from './Registration.module.sass';
import OutlinedInput from "../../components/OutlinedInput";
import Button from "../../components/Button";
import { Colors } from "../../utils/Colors";
import { generateSeedPhrase } from "near-seed-phrase";

const regExpression = /^[a-zA-Z0-9!#$%^&*()_+\-=\[\]{};':"\\|<>.\/?]*$/

const Registration = ({ contract, currentUser, nearConfig }) => {
    // const alert = useAlert();

    const [accountID, setAccountID] = useState<string>('');
    const [isValid, setIsValid] = useState<boolean>(false)

    const validate = useCallback(() => {
        let isValid = true;
        const checkAccountIDReg = regExpression.test(accountID)

        if (!checkAccountIDReg || accountID.length < 2) {
            isValid = false
        }

        setIsValid(isValid);
    },[accountID])

    useEffect(() => {
        validate()
    }, [validate])

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { value } = e.target;
        setAccountID(value);
    }

    const submitAccountID = async () => {
      try {
        setIsValid(false);
        // const keyPair = KeyPair.fromRandom('ed25519');
        // const publicKey = keyPair.getPublicKey().toString();

        const { seedPhrase, publicKey } = generateSeedPhrase();
        console.log('Generate seedPhrase: ', seedPhrase);
        console.log('Generate publicKey: ', publicKey);

        const newAccount =  {
          account_id: accountID,
          public_key: publicKey
        };

        console.log('nearConfig.NEXT_PUBLIC_CONTRACT_NAME_MAKE_WALLET', nearConfig.NEXT_PUBLIC_CONTRACT_NAME_MAKE_WALLET);
        console.log('nearConfig', nearConfig);

        const args = {
          "tags": [{ "person": "Raf", "company": "4ire", "purpose": "test"}],
          "contract_name": nearConfig.CONTRACT_NAME_MAKE_WALLET,
          "args": { "function_name": "make_wallet", "params": JSON.stringify({"network": "testnet", "new_account": newAccount}) }
        };

        const result = await contract.call(args, nearConfig.GAS, nearConfig.ATTACHED_DEPOSIT_AMOUNT);

        setIsValid(true);

        if (result){
          console.log("created success");


        } else {
            console.log('result', result);
        }


        // callContract(currentUser.account, nearConfig, 'app', args)
        //   .then((txid) => {
        //     console.log(txid);
        //     const link = `https://explorer.${nearConfig.networkId}.near.org/transactions/${txid}`;
        //     alert.show(link);
        //
        //     // currentUser.accountId = accountID;
        //     // account = await near.account(currentUser.accountId);
        //
        //   })
        //   .finally(() => {setIsValid(true)})

      } catch(error) {
        console.log("Error: ", error);
        // alert.error(error);
      }
    }

    return (
        <div className={ styles.root }>
            <div className={ styles.baseBody }>
                <div className={ styles.registrationUnderBody2 }>
                  <div>Welcome, {currentUser.accountId}!</div>
                  <div>Your balance: {currentUser.balance} Ⓝ</div>
                </div>
                <hr className={ styles.horizontalLine2 } />

                <div className='flexInlineCenter textHeader padding-10-10-0'>
                    {'Create NEAR account'}
                </div>
                <hr className={ styles.horizontalLine1 } />
                <div className={ styles.registrationSubBody0 }>
                    {'Enter an Account ID to use with your NEAR account. Your Account ID will be used for all NEAR operations, ' +
                    'including sending and receiving assets.\n'}
                </div>
                <div className='flexInlineCenter'>
                     <OutlinedInput onChange={ handleChange } label={'ACCOUNT ID'} />
                </div>
                <div className={ styles.registrationSubBody1 }>
                    {'Your account ID can contain any of the following:\n'}
                </div>

                <ul className={ styles.registrationListBody1 }>
                    <li>{ 'Lowercase characters (a-z)'}</li>
                    <li>{'Digits (0-9)'}</li>
                    <li>{'Characters (_-) can be used as separators'}</li>
                </ul>
                <div className={ styles.registrationSubBody3 }>
                    {'Your account ID CANNOT contain:'}
                </div>
                <ul className={ styles.registrationListBody2 }>
                    <li>{'Characters "@" or "."'}</li>
                    <li>{'Fewer than 2 characters'}</li>
                    <li>{'More than 64 characters (including .near)'}</li>
                </ul>
                <div className='flexInlineCenter'>
                    <Button
                        onClick={ submitAccountID }
                        text='Create'
                        textColor={ Colors.white }
                        backgroundColor={ Colors.blue }
                        disabled={ !isValid }
                    />
                </div>
                <div className={ styles.registrationUnderBody1 }>
                    {'By creating a NEAR account, you agree to the NEAR Wallet'} &#160;
                    <a
                      href="https://wallet.testnet.near.org/20210319-near-tcs.a776bb61.pdf"
                      className={ styles.registrationLink }>
                      {'Terms of Service'}
                    </a>
                  &#160;{'and'} &#160;
                    <a
                      href="https://near.org/privacy/"
                      className={ styles.registrationLink }>
                      {'Privacy Policy.'}
                    </a>
                </div>
            </div>
        </div>
    )
}

export default Registration;
