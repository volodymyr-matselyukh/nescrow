import usePageNavigationStore from '@/store/pageNavigationStore';
import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { FC, ReactNode } from 'react';

interface Props {
  children?: ReactNode;
  text?: string;
  href: string;
  className?: string;
}

const EscrowLink: FC<Props> = ({ text, href, className, children }) => {
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
      {text ?? children}
    </Link>
  );
};

export default EscrowLink;
