const Header = () => {
  return (
    <div>
      <h1 className="text-xl">Marketplace stats:</h1>

      <div>
        <p>
          <span>Total deposit:</span>
          {" "}
          <span>$500</span>
        </p>

        <p>
          <span>Current user:</span>
          {" "}
          <span>vova</span>
        </p>

        <p>
          <span>Current user deposit:</span>
          {" "}
          <span>$300</span>
        </p>
      </div>
    </div>
  );
};

export default Header;
