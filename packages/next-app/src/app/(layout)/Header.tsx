'use client';

import EscrowLink from '@/components/EscrowLink';
import usePageNavigationStore from '@/store/pageNavigationStore';
import { Spin } from 'antd';
import { usePathname } from 'next/navigation';
import { useEffect } from 'react';
import ConnectWallet from './ConnectWallet';

const Header = () => {
  const { isNavigating, setIsNavigating } = usePageNavigationStore();

  const pathname = usePathname();

  useEffect(() => {
    if (isNavigating) {
      setIsNavigating(false);
    }
  }, [pathname]);

  return (
    <div className="flex items-center justify-between p-5">
      <EscrowLink href="/home">
        <span className="mb-2 block w-[120px] p-4 text-xl uppercase outline-dashed outline-2 outline-black hover:bg-gray-500 hover:text-white">
          Need of escrow
        </span>
      </EscrowLink>

      {isNavigating && (
        <span className="text-gray-300">
          Taking you somewhere else...
          <Spin />
        </span>
      )}

      <ConnectWallet />
    </div>
  );
};

export default Header;
