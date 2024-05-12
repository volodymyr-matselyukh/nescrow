import DepositFundsForm from "./form/DepositFundsForm";

const Page = () => {

  return (
    <div className="w-[500px] mx-auto p-5 mt-10 flex flex-col">
      <h1 className='mb-4 text-xl'>Deposit funds</h1>

          <hr className='mb-4' />

      <DepositFundsForm />

    </div>
  )
}

export default Page;
