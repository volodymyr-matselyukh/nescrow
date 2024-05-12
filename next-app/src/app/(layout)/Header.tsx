import Link from 'next/link';
import SignIn from './SignIn';

const Header = () => {
  return (
    <div className='flex justify-between p-5 items-center'>
      <Link href="/home">
        <span className="mb-2 block w-[120px] p-4 text-xl uppercase hover:bg-gray-500 hover:text-white outline-2 outline-black outline-dashed">
          Need of escrow
        </span>
      </Link>

      <SignIn />
    </div>
  );
};

export default Header;
