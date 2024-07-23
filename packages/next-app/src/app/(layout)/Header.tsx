'use client';

import EscrowLink from '@/components/EscrowLink';
import usePageNavigationStore from '@/store/pageNavigationStore';
import { Spin } from 'antd';
import { usePathname } from 'next/navigation';
import { useEffect } from 'react';
import ConnectedWallet from './ConnectedWallet';
import LoggedInUser from './LoggedInUser';

const Header = () => {
  const { isNavigating, setIsNavigating } = usePageNavigationStore();

  const pathname = usePathname();

  useEffect(() => {
    if (isNavigating) {
      setIsNavigating(false);
    }
  }, [pathname]);

  return (
    <div className="flex items-center justify-between px-5">
      <div className='bg-gray-200 stretch h-full justify-center flex flex-col'>
        <EscrowLink href="/home">
          <span className="mb-2 block w-[200px] rounded-lg py-4 text-center text-xl uppercase outline-dashed outline-2 outline-black hover:bg-gray-500 hover:text-white">
            Need of escrow
          </span>
        </EscrowLink>
      </div>

      {isNavigating && (
        <span className="text-gray-300">
          Taking you somewhere else...
          <Spin />
        </span>
      )}

      <div className="pt-5 flex flex-col items-end gap-2">
        <LoggedInUser />
        <ConnectedWallet />
      </div>
    </div>
  );
};

export default Header;
