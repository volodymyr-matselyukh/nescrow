import ResetPassword from './components/ResetPassword';

const Page = ({ searchParams }: { searchParams: { message: string } }) => {
  return (
    <div className="w-full">
      <h1 className="mb-4 text-xl">Forgot password</h1>

      <hr className="mb-4" />

      <div className="ml-auto mr-0 w-full">
        <ResetPassword />
      </div>

      {searchParams?.message && (
        <p className="text-foreground rounded-lg bg-gray-300 p-4 text-center mt-5">
          {searchParams.message}
        </p>
      )}
    </div>
  );
};

export default Page;
