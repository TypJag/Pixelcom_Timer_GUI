import React from 'react'
import styled from 'styled-components'
import { HStack, VStack } from 'react-stacked'
import { FaChevronUp, FaChevronDown } from 'react-icons/fa'

const BigNumber = styled.div`
  font-size: 120px;
  font-weight: 700;
  color: #333;
  display: flex;
  justify-content: center;
  flex-grow: 1;
  align-items: center;
`

const Button = styled.button`
  display: flex;
  flex-grow: 1;
  justify-content: center;
  align-items: center;
  font-size: 24px;
  font-weight: 700;
  background-color: #fff;
  border: none;
  border-radius: 8px;
  width: 100px;
`

interface ChevronButtonProps {
  value: number
  onChange: (value: number) => void
}

export const ChevronButton: React.FC<ChevronButtonProps> = ({ value, onChange }) => {
  const Icon = value > 0 ? FaChevronUp : FaChevronDown

  return (
    <Button onClick={() => onChange(value)}>
      <VStack justifyContent='center' alignItems='center' gap={8}>
        {value > 0 && <Icon />}
        <p>{value > 0 && '+'}{value}</p>
        {value < 0 && <Icon />}
      </VStack>
    </Button>
  )
}

const TimeLeft: React.FC = () => {
  const [timeLeft, setTimeLeft] = React.useState(0)

  return (
    <HStack grow={1}>
      <p>Time Left</p>
      <BigNumber>{timeLeft}</BigNumber>
      <VStack gap={8}>
        <ChevronButton value={20} onChange={(value) => setTimeLeft(timeLeft + value)} />
        <ChevronButton value={-20} onChange={(value) => setTimeLeft(timeLeft + value)} />
      </VStack>
    </HStack>

  )
}

export default TimeLeft
