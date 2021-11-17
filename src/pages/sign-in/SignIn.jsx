import React from 'react';
import styles from "../registration/Registration.module.sass";

export default function SignIn() {
  return (
    <>
      <p className={ styles.registrationSubBody0 }>
          This app demonstrates a key element of NEAR’s UX & NEAR Apps contract invocations:
          once an app has permission to make calls on behalf of a user (that is, once a user
          signs in), the app can make calls to the blockhain for them without prompting
          extra confirmation.
      </p>
      <p className={ styles.registrationSubBody0 }>
          Go ahead and sign in to try it out!
      </p>
    </>
  );
}
