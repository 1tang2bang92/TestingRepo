import React from 'react';
import { render, screen } from '@testing-library/react';
import Router from '../Router';

test('renders learn react link', () => {
  render(<Router />);
  const linkElement = screen.getByText(/friends/i);
  expect(linkElement).toBeInTheDocument();
});
