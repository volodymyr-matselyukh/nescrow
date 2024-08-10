import DepositFundsForm from './form/DepositFundsForm';

const Page = () => {
  return (
    <div>
      <h1 className="mb-4 text-xl">Deposit funds</h1>

      <hr className="mb-4" />

      <div className="ml-auto mr-0 max-w-[400px]">
        <DepositFundsForm />
      </div>
    </div>
  );
};

export default Page;
