import WithdrawFundsForm from "./form/WithdrawFundsForm";

const Page = () => {
  return (
    <div>
      <h1 className="mb-4 text-xl">Withdraw funds</h1>

      <hr className="mb-4" />

      <div className="ml-auto mr-0 max-w-[400px]">
        <WithdrawFundsForm />
      </div>
    </div>
  );
};

export default Page;
