'use client';

import EscrowLink from '@/components/EscrowLink';
import React from 'react';

const LeftNavigation = () => {
  return (
    <menu className="flex basis-[200px] flex-col">
      <EscrowLink href="/home" className="p-2 hover:bg-blue-400 hover:text-white" text="Home" />
      <EscrowLink href="/deposit" className="p-2 hover:bg-blue-400 hover:text-white" text="Deposit funds" />
    </menu>
  );
};

export default LeftNavigation;
