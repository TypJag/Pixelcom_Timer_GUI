import React, { useState } from 'react'
import './App.css'
import ConnectionButtons from './components/ConnectionButtons'
import Flags from './components/Flags'
import { HStack, VStack } from 'react-stacked'
import TimeLeft from './components/TimeLeft'
import DefaultTime from './components/DefaultTime'

type Page = 'home' | 'settings'

interface NavigationProps {
  navigate: (page: Page) => void
}

const Home = ({ navigate }: NavigationProps): React.ReactElement => {
  return (
    <HStack height='100vh' backgroundColor='#eee' padding={8} gap={8}>
      <VStack grow={1} gap={8}>
        <HStack gap={8}>
          <TimeLeft />
          <DefaultTime />
        </HStack>
        <button onClick={() => navigate('settings')}>Go to settings</button>
        <ConnectionButtons />
      </VStack>

      <Flags />
    </HStack>
  )
}

const Settings = ({ navigate }: NavigationProps): React.ReactElement => {
  return (
    <div>
      <button onClick={() => navigate('home')}>Go to home</button>
      <h1>Settings</h1>
    </div>
  )
}

function App (): React.ReactElement {
  const [page, setPage] = useState<Page>('home')

  if (page === 'settings') {
    return (
      <Settings navigate={setPage} />
    )
  }

  return (
    <Home navigate={setPage} />
  )
}

export default App
