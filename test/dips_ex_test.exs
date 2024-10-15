defmodule DipsExTest do
  use ExUnit.Case
  doctest DipsEx

  test "test" do
    assert DipsEx.init_model("native/rust/models/electra-small-6-layers-q4_0.gguf") == {}
    assert DipsEx.run_model("阿張先生嗰時好nice㗎", "dips_str") == ["阿-張|先生 嗰-時 好 nice 㗎"]
    assert DipsEx.run_model("阿張先生嗰時好nice㗎", "fine") == ["阿", "張", "先生", "嗰", "時", "好", "nice", "㗎"]
    assert DipsEx.run_model("阿張先生嗰時好nice㗎", "coarse") == ["阿張先生", "嗰時", "好", "nice", "㗎"]
    assert DipsEx.run_model("阿張先生嗰時好nice㗎", "dips") == ["S", "D", "P", "I", "S", "D", "S", "S", "I", "I", "I", "S"]
    assert DipsEx.free_model() == {}
  end
end
