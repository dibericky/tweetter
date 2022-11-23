import React from 'react';
import Header from './Header';
import Profile from './profile';

function App() {
  return (
    <main className="flex justify-around">
      <div className="grow justify-end flex align-end">
        <Header />
      </div>
      <div className="grow">
        <Profile />
      </div>
    </main>
  );
}

export default App;
