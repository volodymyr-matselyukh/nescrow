import React from 'react';

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <div className="mx-auto mt-10 flex w-[500px] flex-col p-5">{children}</div>
  );
}
