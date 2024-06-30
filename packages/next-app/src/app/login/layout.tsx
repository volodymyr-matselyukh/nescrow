import React from 'react';
import '../globals.css';
import PublicHeader from './PublicHeader';

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <>
      <PublicHeader />
      <div className="flex justify-center">{children}</div>
    </>
  );
}
