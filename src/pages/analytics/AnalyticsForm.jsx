import React, { useState, useEffect } from 'react';
import PropTypes from 'prop-types';
import { encode_utf8_base64, callContract } from './utils/CallContract';

Form.propTypes = {
  onSubmit: PropTypes.func.isRequired,
  currentUser: PropTypes.shape({
    accountId: PropTypes.string.isRequired,
    balance: PropTypes.string.isRequired
  }),
  analytics: PropTypes.shape({
    app_id: PropTypes.string.isRequired,
    action_id: PropTypes.string.isRequired,
    user_name: PropTypes.string.isRequired
  })
};

export default function AnalyticsForm({ onSubmit, currentUser, analytics, disabled }) {
  const [formData, setFormData] = useState(analytics);

  const handleChange = (e) => {
    setFormData(data => ({...data,
      [e.target.name]: e.target.value
    }))
  }

  const handleSubmit = (e) => {
    e.preventDefault()
    onSubmit(formData)
  }

  useEffect(() => {
    setFormData(analytics)
  }, [analytics])

  let initData = {
    app_id: 'Example App Log',
    action_id: 'Example Action Log',
    user_name: currentUser ? currentUser.accountId : 'Log SuperHero'
  };

  const [analytics, setAnalytics] = useState(initData);

  const onSubmit = (formData) => {
    setIsSubmitting(true);

    // either you can call `contract.log_analytics(args);`
    // but this will not return TransactionId, cause doesn't specified in contract `contract\src\lib.rs`

    try {
      const args = {"encoded": encode64( formData )};

      callContract(account, nearConfig, 'log_analytics', args)
        .then((txid) => {
          setIsSubmitting(false)

          const link = `https://explorer.${nearConfig.networkId}.near.org/transactions/${txid}`;
          alert.show(link);
        })

    } catch(error) {
      alert.error(error);
    }
  };

  const encode64 = (formData) => {
    return encode_utf8_base64(formData.app_id) + '_' +
      encode_utf8_base64(formData.action_id) + '_' +
      encode_utf8_base64(formData.user_name);
  }

  return (
    <form>
      <fieldset id="fieldset" disabled={disabled}>
        <p>Sign into log analytics, {currentUser.accountId}!</p>
        
        <p>
          <label htmlFor="app_id">App #</label>
          <input
            autoComplete="off"
            autoFocus
            name="app_id"
            id="app_id"
            value={ formData.app_id }
            onChange={handleChange}
            required />
        </p>
        <p>
          <label htmlFor="action_id">Action #</label>
          <input
            autoComplete="off"
            name="action_id"
            id="action_id"
            value={ formData.action_id }
            onChange={handleChange}
            required />
        </p>
        <p>
          <label htmlFor="user_name">User name</label>
          <input
            autoComplete="off"
            name="user_name"
            id="user_name"
            value={ formData.user_name }
            onChange={handleChange}
            required />
        </p>

        <button type="submit" onClick={handleSubmit}>
          Submit
        </button>
      </fieldset>
    </form>
  );
}
