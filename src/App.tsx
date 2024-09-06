import './App.css'
import ConnectionButtons from './components/ConnectionButtons'
import Flags from './components/Flags'
import { HStack } from 'react-stacked'

function App (): React.ReactElement {
  return (

    <HStack height='100vh'>
      <ConnectionButtons />
      <Flags />
    </HStack>
  )
}

export default App
