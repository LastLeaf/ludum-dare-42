var fs = require('fs-extra')
var execFile = require('child_process').execFile
var gulp = require('gulp')
var concat = require('gulp-concat')
var sourcemaps = require('gulp-sourcemaps')
var webpack = require('webpack')

var libCompilerConfig = require('./glayout/lib/webpack.config')

var execShellScript = function(path, cwd, cb) {
  execFile('sh', [path], {cwd: cwd}, function(err, stdout, stderr) {
    if (err) {
      cb(err)
      return
    }
    cb()
  })
}

var convertSourceMapPath = function() {
  return sourcemaps.mapSources(function(sourcePath, file) {
    if (sourcePath.match(/^webpack:\/\/__glayoutLib__\/webpack\//)) {
      return sourcePath.replace('webpack://__glayoutLib__/webpack/', '__internal__/webpack/')
    }
    if (sourcePath.match(/^webpack:\/\/__glayoutLib__\/lib\//)) {
      return sourcePath.replace('webpack://__glayoutLib__/lib/', 'lib/src/')
    }
    if (sourcePath.match(/^..\//)) {
      return sourcePath.replace(/^(..\/)+/, '__internal__/')
    }
    if (sourcePath.match(/^src\//)) {
      return sourcePath.replace('src/', 'asm/src/')
    }
    return '__internal__/' + sourcePath
  })
}

var toUnderlineCase = function(str) {
  return str.replace(/[A-Z]+/g, function(c) {
    return '_' + c.toLowerCase()
  })
}

var generateLibInterfaces = function(path) {
  var slices = ['mergeInto(LibraryManager.library, {']
  // eslint-disable-next-line
  var lib = (new Function(fs.readFileSync(path, {encoding: 'utf8'}) + '\n; return __glayoutLib__'))()
  for (var k in lib) {
    var func = lib[k]
    if (typeof func === 'function') {
      var argsLength = func.length
      var funcName = k
      var argList = ''
      for (var i = 0; i < argsLength; i++) {
        if (i) argList += ', '
        argList += 'arg' + i
      }
      slices.push(toUnderlineCase(funcName) + ': function(' + argList + ') { return window.__glayoutLib__.' + funcName + '(' + argList + ') },')
    }
  }
  slices.push('})')
  return slices.join('\n')
}

gulp.task('clean-asm', function(cb) {
  execShellScript('scripts/clean.sh', '.', function() {
    execShellScript('scripts/clean.sh', 'glayout/asm', cb)
  })
})

gulp.task('clean-lib', function(cb) {
  fs.remove('./glayout/lib/bin', cb)
})

gulp.task('clean', ['clean-asm', 'clean-lib'], function(cb) {
  fs.remove('./bin', cb)
})

gulp.task('compile-lib-debug', function(cb) {
  webpack(libCompilerConfig[0]).run(cb)
});

gulp.task('generate-lib-interfaces-debug', ['compile-lib-debug'], function(cb) {
  fs.writeFile('./glayout/lib/bin/interfaces-debug.js', generateLibInterfaces('./glayout/lib/bin/glayout-lib.js'), cb)
})

gulp.task('compile-lib-release', function(cb) {
  webpack(libCompilerConfig[1]).run(cb)
});

gulp.task('generate-lib-interfaces-release', ['compile-lib-release'], function(cb) {
  fs.writeFile('./glayout/lib/bin/interfaces-release.js', generateLibInterfaces('./glayout/lib/bin/glayout-lib.min.js'), cb)
})

gulp.task('compile-asm-debug', ['generate-lib-interfaces-debug'], function(cb) {
  execShellScript('scripts/build-asmjs-debug.sh', '.', cb)
})

gulp.task('compile-asm-release', ['generate-lib-interfaces-release'], function(cb) {
  execShellScript('scripts/build-asmjs-release.sh', '.', cb)
})

gulp.task('build-release', ['compile-asm-release'], function(cb) {
  return gulp.src(['./glayout/lib/bin/glayout-lib.min.js', './target/asmjs-unknown-emscripten/release/src.js'])
    .pipe(concat('glayout.min.js'))
    .pipe(gulp.dest('./bin/'))
})

gulp.task('build-debug-with-sourcemap', ['compile-asm-debug'], function(cb) {
  return gulp.src(['./glayout/lib/bin/glayout-lib.js', './target/asmjs-unknown-emscripten/debug/deps/src-*.js'])
    .pipe(sourcemaps.init({loadMaps: true}))
    .pipe(concat('glayout.js'))
    .pipe(convertSourceMapPath())
    .pipe(sourcemaps.write('.', {includeContent: true}))
    .pipe(gulp.dest('./bin/'))
})

gulp.task('build-debug', ['compile-asm-debug'], function(cb) {
  return gulp.src(['./glayout/lib/bin/glayout-lib.js', './target/asmjs-unknown-emscripten/debug/deps/src-*.js'])
    .pipe(sourcemaps.init({loadMaps: true}))
    .pipe(concat('glayout.js'))
    .pipe(convertSourceMapPath())
    .pipe(sourcemaps.write('.', {includeContent: false, sourceRoot: '..'}))
    .pipe(gulp.dest('./bin/'))
})

gulp.task('watch', ['build-debug'], function() {
  gulp.watch(['glayout/asm/src/*.rs', 'glayout/asm/src/**/*.rs', 'glayout/lib/*.js',  'glayout/lib/src/*.js', 'glayout/lib/src/**/*.js', 'glayout/lib/src/**/*.glsl', 'src/**/*.rs'], ['build-debug'])
    .on('change', function(event) {
      console.log('file: ' + event.path + ' was ' + event.type);
    })
})

gulp.task('default', ['build-release'])
