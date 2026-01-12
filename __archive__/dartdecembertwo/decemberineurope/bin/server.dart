import 'dart:io';

import 'package:shelf/shelf.dart';
import 'package:shelf/shelf_io.dart' as io;
import 'package:shelf_static/shelf_static.dart';

void main() async {
  final staticFilesPath = 'web';

  final staticHandler = createStaticHandler(
    staticFilesPath,
    defaultDocument: 'index.html',
  );

  final handler = const Pipeline().addMiddleware(logRequests()).addHandler((
    Request request,
  ) {
    if (request.method == 'GET' && request.url.path == 'lorem') {
      return Response.ok('ipsum');
    }

    return staticHandler(request);
  });

  final port = 8080;
  final server = await io.serve(handler, InternetAddress.anyIPv4, port);

  print('December in Europe :: http://${server.address.host}:${server.port}');
}
