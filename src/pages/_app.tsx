import 'regenerator-runtime/runtime';
import React, { useState, useEffect } from 'react';
import Button from "../components/Button";
import RegistrationForm from "./registration/Registration";
import SignIn from "./sign-in/SignIn";
import {observer} from "mobx-react";
import {AuthStore, NearConfig, UserType} from "../stores/AuthStore";
import {Contract, WalletConnection} from "near-api-js";
import '../global.scss';

const MyApp = observer(() => {
    const { tryToConnect } = AuthStore;
    const [nearConfig, setNearConfig] = useState<NearConfig>();
    const [contract, setContract] = useState<Contract>();
    const [currentUser, setCurrentUser] = useState<UserType>();
    const [wallet, setWallet] = useState<WalletConnection>();

    useEffect(() => {
        tryToConnect().then((res: any) => {
            setNearConfig(res.nearConfig);
            setContract(res.contract)
            setCurrentUser(res.currentUser);
            setWallet(res.wallet);

            console.log('currentUser', res.currentUser);
        });
    }, [])

    const signIn = () => {
        wallet.requestSignIn(
            nearConfig.contractName,
            'NEAR Apps'
        );
    };

    const signOut = () => {
        wallet.signOut();
        window.location.replace(window.location.origin + window.location.pathname);
    };

    return (
        <main>
            <header>
                <h1>NEAR Apps</h1>
                { currentUser
                    ? <Button onClick={signOut} text={'Log out'}/>
                    : <Button onClick={signIn} text={'Log in'}/>
                }
            </header>
            { currentUser
                ? <RegistrationForm contract={contract} currentUser={currentUser} nearConfig={nearConfig} />
                : <SignIn/>
            }
        </main>
    );
})

export default MyApp

// MyApp.propTypes = {
//     contract: PropTypes.shape({
//         call: PropTypes.func.isRequired,
//         log_analytics: PropTypes.func.isRequired
//     }).isRequired,
//     currentUser: PropTypes.shape({
//         accountId: PropTypes.string.isRequired,
//         balance: PropTypes.number.isRequired
//     }),
// // analytics: PropTypes.shape({
// //   app_id: PropTypes.string.isRequired,
// //   action_id: PropTypes.string.isRequired,
// //   user_name: PropTypes.string.isRequired
// // }),
//     nearConfig: PropTypes.shape({
//         contractName: PropTypes.string.isRequired
//     }).isRequired,
//     wallet: PropTypes.shape({
//         requestSignIn: PropTypes.func.isRequired,
//         signOut: PropTypes.func.isRequired
//     }).isRequired
// };