'use client';

import EscrowLink from '@/components/EscrowLink';
import Logo from '@/components/Logo';
import usePageNavigationStore from '@/store/pageNavigationStore';
import { Spin } from 'antd';
import { usePathname } from 'next/navigation';
import { useEffect } from 'react';

const PublicHeader = () => {
  const { isNavigating, setIsNavigating } = usePageNavigationStore();

  const pathname = usePathname();

  useEffect(() => {
    if (isNavigating) {
      setIsNavigating(false);
    }
  }, [pathname]);

  return (
    <div className="items-middle flex items-center justify-between">
      <div className="flex h-[148px] w-[220px] items-center bg-gray-200 p-5">
        <EscrowLink href="/login">
          <Logo />
        </EscrowLink>
      </div>

      {isNavigating && (
        <span className="text-gray-300">
          Taking you somewhere else...
          <Spin />
        </span>
      )}

      {/* placeholder */}
      <div className="w-52"></div>
    </div>
  );
};

export default PublicHeader;
