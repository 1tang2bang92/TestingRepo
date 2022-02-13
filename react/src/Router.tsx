import React from 'react'
import { BrowserRouter, Route, Routes } from "react-router-dom"
import Friends from './pages/Friends'
import Chat from './pages/Chat'

function Router() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Friends />} />
        <Route path="/chat" element={<Chat />} />
      </Routes>
    </BrowserRouter>
  )
}

export default Router;
