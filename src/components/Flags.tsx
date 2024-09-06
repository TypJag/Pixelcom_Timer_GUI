import React from 'react'
import styled from 'styled-components'
import { VStack } from 'react-stacked'

const Button = styled.button<{ color: string, pattern?: 'checkered' | null }>`
  background-color: ${props => props.color};
  border: none;
  width: 100%;
  font-size: 20px;
  font-weight: bold;
  color: white;
  height: 100%;

  background-image: ${props => props.pattern === 'checkered' ? 'linear-gradient(45deg, #000000 25%, transparent 25%), linear-gradient(-45deg, #000000 25%, transparent 25%), linear-gradient(45deg, transparent 75%, #000000 75%), linear-gradient(-45deg, transparent 75%, #000000 75%)' : 'none'};
  background-size: 40px 40px;
  background-position: 0 0, 0 20px, 20px -20px, -20px 0px;

  border-radius: 8px;
`

const Flags: React.FC = () => {
  const changeFlag = (flag: string): void => {
    console.log(`Flag changed to ${flag}`)
    // Add rust call here
  }

  return (
    <VStack width={200} justifyContent='stretch' alignItems='stretch' height='100%' gap={8}>
      <Button color='grey' onClick={() => changeFlag('noFlag')}>No Flag</Button>
      <Button color='red' onClick={() => changeFlag('red')} />
      <Button color='green' onClick={() => changeFlag('green')} />
      <Button color='yellow' onClick={() => changeFlag('yellow')} />
      <Button color='white' pattern='checkered' onClick={() => changeFlag('finish')} />
      <Button color='grey' onClick={() => changeFlag('showTime')}>Show Time</Button>
    </VStack>
  )
}

export default Flags
