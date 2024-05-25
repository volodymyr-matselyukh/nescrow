import usePageNavigationStore from '@/store/pageNavigationStore';
import Link from 'next/link';
import { usePathname } from 'next/navigation';
import React, { FC } from 'react';

interface Props {
  text: string;
  href: string;
  className: string;
}

const EscrowLink: FC<Props> = ({ text, href, className }) => {
  const { setIsNavigating } = usePageNavigationStore();
  const pathname = usePathname();

  return (
    <Link
      href={href}
      className={className}
      onClick={() => {
        if (pathname !== href) {
          setIsNavigating(true);
        }
      }}
    >
      {text}
    </Link>
  );
};

export default EscrowLink;
