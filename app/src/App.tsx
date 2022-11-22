import React from 'react';
import Header from './Header';
import Profile from './profile';

function App() {
  return (
    <main className="flex">
      <div>
        <Header />
      </div>
      <div className="grow">
        <Profile nickname={"ricky"} />
      </div>
    </main>
  );
}

export default App;
