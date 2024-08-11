'use client';

import EscrowLink from '@/components/EscrowLink';
import usePageNavigationStore from '@/store/pageNavigationStore';
import { Spin } from 'antd';
import { usePathname } from 'next/navigation';
import { useEffect } from 'react';
import ConnectedWallet from './ConnectedWallet';
import LoggedInUser from './LoggedInUser';
import Logo from '@/components/Logo';

const Header = () => {
  const { isNavigating, setIsNavigating } = usePageNavigationStore();

  const pathname = usePathname();

  useEffect(() => {
    if (isNavigating) {
      setIsNavigating(false);
    }
  }, [pathname]);

  return (
    <div className="flex items-center justify-between pr-5">
      <div className="stretch flex h-full flex-col justify-center bg-gray-200 px-5 w-[220px]">
        <EscrowLink href="/home">
          <Logo />
        </EscrowLink>
      </div>

      {isNavigating && (
        <span className="text-gray-300">
          Taking you somewhere else...
          <Spin />
        </span>
      )}

      <div className="flex flex-col items-end gap-2 pt-5">
        <LoggedInUser />
        <ConnectedWallet />
      </div>
    </div>
  );
};

export default Header;
