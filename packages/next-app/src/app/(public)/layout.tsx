import React from 'react';
import '../globals.css';
import PublicHeader from './login/PublicHeader';

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <>
      <PublicHeader />
      <div className="flex grow justify-between gap-5">
        <div className="w-[220px] bg-gray-200 shrink-0"></div>
        <div className="flex justify-center grow">{children}</div>
        <div className="w-[220px] shrink-0"></div>
      </div>
    </>
  );
}
