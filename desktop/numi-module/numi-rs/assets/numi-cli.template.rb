class {class} < Formula
  desc "Smart calculator"
  homepage "https://numi.app"
  if OS.mac?
    if Hardware::CPU.intel?
      url "{mac_intel_url}"
      sha256 "{mac_intel_sha}"
    elsif Hardware::CPU.arm?
      url "{mac_arm_url}"
      sha256 "{mac_arm_sha}"
    end
  elsif OS.linux?
    if Hardware::CPU.intel?
      url "{linux_intel_url}"
      sha256 "{linux_intel_sha}"
    elsif Hardware::CPU.arm?
      url "{linux_arm_url}"
      sha256 "{linux_arm_sha}"
    end
  end
  def install
    bin.install "numi-cli"
  end
end
