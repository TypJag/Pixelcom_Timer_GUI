import React from 'react'
import styled from 'styled-components'
import { HStack, VStack } from 'react-stacked'
import { ChevronButton } from './TimeLeft'

const BigNumber = styled.div`
  font-size: 56px;
  font-weight: 700;
  color: #333;
  display: flex;
  justify-content: center;
  flex-grow: 1;
  align-items: center;
`

const DefaultTime: React.FC = () => {
  const [timeLeft, setTimeLeft] = React.useState(0)

  return (
    <HStack grow={1}>
      <p>Default Time</p>
      <BigNumber>{timeLeft}</BigNumber>
      <VStack gap={8}>
        <ChevronButton value={20} onChange={(value) => setTimeLeft(timeLeft + value)} />
        <ChevronButton value={-20} onChange={(value) => setTimeLeft(timeLeft + value)} />
      </VStack>
    </HStack>

  )
}

export default DefaultTime
