'use client';

import Link from 'next/link';
import SignIn from './SignIn';
import { Spin } from 'antd';
import usePageNavigationStore from '@/store/pageNavigationStore';
import { useEffect } from 'react';
import { usePathname } from 'next/navigation';

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
      <Link href="/home">
        <span className="mb-2 block w-[120px] p-4 text-xl uppercase outline-dashed outline-2 outline-black hover:bg-gray-500 hover:text-white">
          Need of escrow
        </span>
      </Link>

      {isNavigating && (
        <span className="text-gray-300">
          Taking you somewhere else...
          <Spin />
        </span>
      )}

      <SignIn />
    </div>
  );
};

export default Header;
