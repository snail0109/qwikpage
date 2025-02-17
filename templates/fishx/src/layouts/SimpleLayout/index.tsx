import React from 'react';

export default (props: any) => (
  <div style={{ textAlign: 'center', marginTop: 200 }}>
    <p>Welcome Login</p>
    {props.children}
  </div>
);
