import React from 'react'
import styled from 'styled-components'
import { HStack } from 'react-stacked'
import { invoke } from '@tauri-apps/api/tauri'

const Button = styled.button`
  background-color: #4CAF50;
  width: 100%;
  border: none;
  color: white;
  padding: 15px 32px;
  text-align: center;
  text-decoration: none;
  font-size: 16px;
  cursor: pointer;
`

const ConnectionButtons: React.FC = () => {
  function connect (): void {
    invoke('connect', { host: '127.0.0.1', port: 24 }).then((res) => {
      console.log(res)
    }).catch((err) => {
      console.error(err)
    })
  }

  function disconnect (): void {
    invoke('disconnect', {}).then((res) => {
      console.log(res)
    }).catch((err) => {
      console.error(err)
    })
  }

  function sendTime (): void {
    invoke('send_time', { time: 100 }).then((res) => {
      console.log(res)
    }).catch((err) => {
      console.error(err)
    })
  }

  return (
    <HStack gap={2} padding={2} grow={1}>
      <Button onClick={connect}>Connect</Button>
      <Button onClick={disconnect}>Disconnect</Button>
      <Button onClick={sendTime}>Send time</Button>
    </HStack>

  )
}

export default ConnectionButtons
