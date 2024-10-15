defmodule DipsEx do
  use Rustler, otp_app: :dips_ex, crate: "rust"
  # When your NIF is loaded, it will override this function.
  def init_model(_path_model), do: :erlang.nif_error(:nif_not_loaded)
  def run_model(_input, _mode), do: :erlang.nif_error(:nif_not_loaded)
  def free_model(), do: :erlang.nif_error(:nif_not_loaded)
end
