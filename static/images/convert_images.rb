#!/usr/bin/env ruby

images = Dir["./**/*.png"] + Dir["./**/*.jpg"]

for image in images
  target = File.dirname(image) + "/" + File.basename(image, '.*') + '.webp'
  next if File.exists?(target)

  system "cwebp -lossless '#{image}' -o '#{target}'"
end
