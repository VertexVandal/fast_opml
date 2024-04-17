defmodule FastOpml do
  use Rustler, otp_app: :fast_opml, crate: "fast_opml"

  # When your NIF is loaded, it will override this function.
  def parse_opml(_a), do: :erlang.nif_error(:nif_not_loaded)

  # defmodule Native do
  #   use Rustler, otp_app: :fast_opml, crate: "fast_opml"

  #   # When your NIF is loaded, it will override this function.
  #   def parse_opml(_a), do: :erlang.nif_error(:nif_not_loaded)
  # end

  # def parse_opml(opml_string), do: FastOpml.Native.parse_opml(opml_string)
end
