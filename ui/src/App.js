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
    name: 'DNA',
    component: <DNA />
  },
  {
    name: 'Algorithms',
    component: <Algorithms />
  },
  {
    name: 'Sequence',
    component: <Sequence />
  },
]

function App() {

  const [screen, setScreen] = React.useState(screens[0]);

  const Button = ({ icon, isClose }) => {
    return <div className={isClose ? 'button close' : 'button'}>
      {icon}
    </div>
  }

  const Header = () => {
    return <div className='window'>
      <div style={{ flex: 8 }} />
      <div className='buttons' >
        <Button icon={<VscChromeMinimize />} />
        <Button icon={<VscChromeMaximize />} />
        <Button isClose icon={<IoMdClose />} />
      </div>
    </div>
  }

  const Tab = ({ component, index, onPress }) => {
    return <div onClick={onPress} className='tab-button'
      style={{
        backgroundColor: component.name === screen.name ? 'blue' : 'transparent',
        color: component.name === screen.name ? 'white' : 'black',
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
        <Header />
        <div className='content'>
          <div style={{ flex: 2, display: 'flex' }}>
            <div style={{ flex: 1 }} />
            <div style={{ flex: 10, display: 'flex', flexDirection: 'column', justifyContent: 'center' }} >
              <h2>DNArchery</h2>
              <div style={{ height: '5vh', width: '80%', display: 'flex', flexDirection: 'row' }} >
                {screens.map((item, index) => {
                  return <Tab key={index} index={index} component={item} onPress={() => setScreen(item)} />
                })}
              </div>
            </div>
            <div style={{ flex: 1 }} />
          </div>
          {screen.component}
          <div style={{ flex: 0.5 }}></div>
        </div>
      </div>
    </IconContext.Provider>
  );
}

export default App;
