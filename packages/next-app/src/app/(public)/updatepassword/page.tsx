import UpdatePassword from './components/UpdatePassword';

const Page = ({
  searchParams,
}: {
  searchParams: { message: string; code: string; error_description: string };
}) => {
  return (
    <div className="w-full">
      <h1 className="mb-4 text-xl">Update password</h1>

      <hr className="mb-4" />

      <div className="ml-auto mr-0 w-full">
        <UpdatePassword code={searchParams.code} />
      </div>

      {searchParams?.message && (
        <p className="text-foreground mt-5 rounded-lg bg-gray-300 p-4 text-center">
          {searchParams.message}
        </p>
      )}

      {searchParams?.error_description && (
        <p className="text-foreground mt-5 rounded-lg bg-gray-300 p-4 text-center">
          {searchParams.error_description}
        </p>
      )}
    </div>
  );
};

export default Page;
