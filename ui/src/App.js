import * as React from 'react';
import { IconContext } from "react-icons";
import { IoMdClose } from 'react-icons/io'
import { VscChromeMaximize, VscChromeMinimize } from 'react-icons/vsc'
import DNA from './screens/DNA';
import Algorithms from './screens/Algorithms';
import Sequence from './screens/Sequence';
import './fonts/Poppins-Regular.ttf';
import './App.css';

const screens = [
  {
    id: '1',
    name: 'Algorithms',
    component: <Algorithms />
  },
  {
    id: '2',
    name: 'DNA',
    component: <DNA />
  },
  {
    id: '3',
    name: 'Sequence',
    component: <Sequence />
  },
]

function App() {

  const [screenIndex, setScreenIndex] = React.useState(0);

  const [screenStates, setScreenStates] = React.useState(
    screens.reduce((acc, screen) => ({ ...acc, [screen.id]: {} }), {})
  );

  const handleTabClick = React.useCallback(
    (index) => {
      setScreenIndex(index);
    },
    []
  );

  const handleScreenChange = React.useCallback(
    (id, state) => {
      setScreenStates((prevState) => ({ ...prevState, [id]: state }));
    },
    []
  );

  const Tab = ({ component, index, onPress }) => {
    return <div onClick={onPress} className='tab-button'
      style={{
        backgroundColor: index === screenIndex ? 'blue' : 'transparent',
        color: index === screenIndex ? 'white' : 'black',
        borderBottomLeftRadius: index === 0 ? '10px' : '0px',
        borderTopLeftRadius: index === 0 ? '10px' : '0px',
        borderBottomRightRadius: index === 2 ? '10px' : '0px',
        borderTopRightRadius: index === 2 ? '10px' : '0px'
      }}>
      <p>{component.name}</p>
    </div>
  }

  return (
    <IconContext.Provider value={{ color: "black", className: "icon" }}>
      <div className="App">
        <div style={{ flex: 1 }} />
        <div className='content'>
          <div style={{ flex: 2, display: 'flex' }}>
            <div style={{ flex: 1 }} />
            <div style={{ flex: 10, display: 'flex', flexDirection: 'column', justifyContent: 'center' }} >
              <h2>DNArchery</h2>
              <div style={{ height: '5vh', width: '80%', display: 'flex', flexDirection: 'row' }} >
                {screens.map((item, index) => {
                  return <Tab key={index} index={index} component={item} onPress={() => handleTabClick(index)} />
                })}
              </div>
            </div>
            <div style={{ flex: 1 }} />
          </div>
          {screens.map((screen, index) => (
            <div key={index} style={{ flex: 7, display: screenIndex === index ? "flex" : "none" }}>
              {React.cloneElement(screen.component, {
                state: screenStates[screen.id],
                onChange: (state) => handleScreenChange(screen.id, state),
              })}
            </div>
          ))}
          <div style={{ flex: 0.5 }}></div>
        </div>
      </div>
    </IconContext.Provider>
  );
}

export default App;
