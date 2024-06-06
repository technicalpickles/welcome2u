#!/usr/bin/ruby

require_relative 'bt'
require 'tempfile'

base_dir = File.dirname(__FILE__)

excluded_modules = [
  "20-uptime",
  "13-public-ip",

  "40-tmux",

  # need to support macos
  "12-ip",
  "31-temperatures",
  "34-services",
  "41-updates",

  # we'll do our own versions of these in 01-intro
  "00-banner"
]

config_path="#{base_dir}/config.sh"

thread_outputs = {}

enabled_modules = Dir.glob("#{base_dir}/modules/*").sort!.delete_if { |f| excluded_modules.include?(File.basename(f)) }
enabled_modules.each do |module_path|
  module_base = File.basename(module_path)
  next if excluded_modules.include?(module_base)

  reader, writer = IO.pipe mode: 'w+'
  module_thread = Thread.new do
    BT.time module_base do
      env = {
        "BASE_DIR" => base_dir,
        "CONFIG_PATH" => config_path,
      }

      spawn env, module_path, out: writer
      Process.wait
      writer.close
    end
  end
  module_thread.name = module_base
  thread_outputs[module_thread] = reader
end


thread_outputs.each do |thread, reader|
  BT.time "join #{thread.name}" do
    thread.join
  end
  puts reader.read
  reader.close
end
