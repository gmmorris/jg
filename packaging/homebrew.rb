class Jg < Formula
  desc "A command-line JSON processor in Rust. jg searches for selector patterns in json input, jg prints each json object that matches a pattern."
  homepage "https://github.com/gmmorris/jg"
  url "https://github.com/gmmorris/jg/archive/0.1.3.osx.tar.gz"
  sha256 "e78ba9e05d8a44d81422b02100f0105c1b24423393b2c68783a0d703859d9caf"
  # depends_on "cmake" => :build

  def install
    # ENV.deparallelize  # if your formula fails when building in parallel
    # Remove unrecognized options if warned by configure
    system "./configure", "--disable-debug",
                          "--disable-dependency-tracking",
                          "--disable-silent-rules",
                          "--prefix=#{prefix}"
    # system "cmake", ".", *std_cmake_args
    system "make", "install" # if this fails, try separate make/make install steps
  end

  test do
    assert_equal "{\"name\":\"jeff goldblum\"}\n", pipe_output("#{bin}/jg .name", '{"name":"jeff goldblum"}')
  end
end