export const sanitize_query = () => {
  sanitize_query_inner(new URLSearchParams(location.search));
};

export const sanitize_query_inner = (search: URLSearchParams) => {
  search.delete("redeem_ready");
  search.delete("totp");
  search.delete("passkey");
  const query = search.toString();
  location.replace(
    `${location.origin}${location.pathname}${query.length ? "?" + query : ""}`
  );
};