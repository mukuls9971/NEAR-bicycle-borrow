import React from 'react';
import PropTypes from 'prop-types';

export default function Form({ onSubmit, currentUser,placeholder }) {
  return (
    <form onSubmit={onSubmit}>
      <fieldset id="fieldset">
        <p className="highlight">
          <label htmlFor="message">Message:</label>
          <input
            autoComplete="off"
            autoFocus
            id="message"
            placeholder={placeholder}
            required
          />
        </p>
        <button type="submit">
          CALL-NEAR
        </button>
      </fieldset>
    </form>
  );
}

Form.propTypes = {
  onSubmit: PropTypes.func.isRequired,
  currentUser: PropTypes.shape({
    accountId: PropTypes.string.isRequired
  })
};
