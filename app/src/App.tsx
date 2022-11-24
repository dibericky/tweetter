import React from 'react';
import { Outlet } from 'react-router-dom';
import Header from './Header';
import Page from './Page';
import Profile from './profile';

function App() {
  return (
    <Page>
        <Outlet />
    </Page>
  );
}

export default App;
