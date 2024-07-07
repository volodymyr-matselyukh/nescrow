'use client';

import EscrowLink from '@/components/EscrowLink';

const LeftNavigation = () => {
  return (
    <menu className="flex flex-col bg-gray-200 self-start">
      <EscrowLink
        href="/home"
        className="p-2 hover:bg-blue-400 hover:text-white"
        text="Home"
      />
      <EscrowLink
        href="/deposit"
        className="p-2 hover:bg-blue-400 hover:text-white"
        text="Deposit funds"
      />
      <EscrowLink
        href="/tasks"
        className="p-2 hover:bg-blue-400 hover:text-white"
        text="Tasks"
      />
    </menu>
  );
};

export default LeftNavigation;
